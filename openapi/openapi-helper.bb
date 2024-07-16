#!/usr/bin/env bb

(require '[clojure.walk :as walk])
(require '[cheshire.core :as json])
(require '[clojure.string :as str])

(require '[babashka.deps :as deps])
(deps/add-deps '{:deps {camel-snake-kebab/camel-snake-kebab {:mvn/version "0.4.2"}}})

(require '[babashka.cli :as cli])
(require '[babashka.fs :as fs])
(require '[camel-snake-kebab.core :as csk])


(defn repascalize
  "Convert tag.SomeResource -> TagSomeResource"
  [form]
  (if (not (string? form))
    form
    (if-let [[whole api noun] (re-find #"([a-zA-Z\d]+)\.([a-zA-Z]+)" form)]
      (str/replace form whole (csk/->PascalCase (str api noun)))
      form)))

(defn emit-matching
  "Only emits a route and OpenAPI spec if tag matches"
  [f [route spec]]
  (when (every? #((fnil str/starts-with? "") (:operationId %) f) (vals spec))
    [route spec]))

(defn collect-refs
  "Collects all of the component strings from OpenAPI tree"
  [form]
  (cond
    (string? form)
    (when (str/starts-with? form "#/components")
      form)

    (map-entry? form)
    form

    (sequential? form)
    (filter string? (flatten form))

    (map? form)
    (filter string? (flatten (vals form)))

    :else
    form))

(defn emit-components
  "Collect referenced schema/parameter/response refs from OpenAPI components"
  [cs components]
  (let [filtered-components (->> cs
                                 ; #/components/kind/name split /
                                 (map #(str/split % #"/"))
                                 ; drop #, components
                                 (map #(map keyword (drop 2 %)))
                                 (map last))] 
    {:schemas    (select-keys (:schemas components) filtered-components)
     :parameters (select-keys (:parameters components) filtered-components)
     :responses  (select-keys (:responses components) filtered-components)}))

(defn emit-subspec
  "Emit a subset of the larger spec"
  [f spec]
  (let [{:keys [components paths] :as parsed-spec} (-> (slurp spec)
                                                       (json/parse-string true))
        paths (if f
                (into {} (keep #(emit-matching f %)) paths)
                paths)
        path-component-refs (set (walk/postwalk collect-refs paths))
        path-components (emit-components path-component-refs components)

        component-refs (set (walk/postwalk collect-refs path-components))
        component-components (emit-components component-refs components)

        components (merge-with merge path-components component-components)]
    (-> parsed-spec
        (assoc :components components)
        (assoc :paths paths)
        (dissoc :tags)
        (dissoc :x-databricks-groups)
        (walk/stringify-keys))))

(defn -main
  [args]
  (let [cli-spec {:spec {:spec {:desc "Input OpenAPI spec"
                                :validate fs/exists?
                                :alias "s"}
                         :filter {:desc "Desired resource to filter out of whole spec"
                                  :alias "f"}}}
        {:keys [filter spec]} (cli/parse-opts args cli-spec)]
    (when-not spec
      (println "Usage\n" (cli/format-opts cli-spec))
      (System/exit 1))
    
    (let [new-spec (walk/postwalk repascalize (emit-subspec filter spec))]
      (println (json/encode new-spec {:pretty true})))))

(-main *command-line-args*)
