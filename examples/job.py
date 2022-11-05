import pyspark
from pyspark.sql import SparkSession
from pyspark.sql.types import StringType
import pyspark.sql.functions as F
from functools import reduce
import requests

spark = SparkSession \
    .builder \
    .master("local[*]") \
    .appName('why-is-the-example-always-wordcount') \
    .getOrCreate()

sc = spark.sparkContext

moby_dick = requests.get('https://www.gutenberg.org/files/2701/old/moby10b.txt').text

words = reduce(
    lambda words, line: words + [w.lower() for w in line.split(' ')], 
    moby_dick.splitlines(),
    []
)

df = spark.createDataFrame(words, StringType())

df \
    .withColumnRenamed(df.columns[0], "word") \
    .groupBy("word") \
    .agg(F.count("word").alias("count")) \
    .orderBy(F.desc("count")) \
    .show()
