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

(defn clean-databricks
  [form]
  (if (map? form)
    (let [to-remove (filter #(str/starts-with? % "x-databricks")
                            (keys form))]
      (apply dissoc form to-remove))
    form))

(defn select-refs
  "Collect referenced schema/parameter/response refs from OpenAPI components"
  [cs components]
  (let [component-keys (->> cs
                            ; #/components/kind/name split /
                            (map #(str/split % #"/"))
                            ; drop #, components
                            (map #(map keyword (drop 2 %)))
                            (map last))]
    {:schemas    (select-keys (:schemas components) component-keys)
     :parameters (select-keys (:parameters components) component-keys)
     :responses  (select-keys (:responses components) component-keys)}))

(defn emit-components-for
  ([all form]
   (emit-components-for all form (set (walk/postwalk collect-refs form))))
  ([all form refs]
   (let [new-form (select-refs refs all)
         new-refs (set (walk/postwalk collect-refs new-form))]
     (if (> (count new-refs) (count refs))
       (emit-components-for all form new-refs)
       new-form))))

(defn emit-subspec
  "Emit a subset of the larger spec"
  [f spec]
  (let [{:keys [components paths] :as parsed-spec} (-> (slurp spec)
                                                       (json/parse-string true))
        paths (if f
                (into {} (keep #(emit-matching f %)) paths)
                paths)

        components (emit-components-for components paths)]
    (-> parsed-spec
        (assoc :components components)
        (assoc :paths paths)
        (dissoc :tags))))

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

    (let [new-spec (->> (emit-subspec filter spec)
                        (walk/stringify-keys)
                        (walk/postwalk repascalize)
                        (walk/postwalk clean-databricks))]
      (println (json/encode new-spec {:pretty true})))))

(-main *command-line-args*)
