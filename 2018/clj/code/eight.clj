(ns code.eight
  (:require [clojure.string :as string]))

(def test-input "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")
(def input (string/trim (slurp "/Users/pavel/hobby/adventofcode/code/resources/day8.txt")))
input

(defn parse [input]
  (map #(Long/parseLong %) (string/split input #" ")))

(def *result (atom 0))

(defn dfs [input]
  (let [[ch met & rst] input]
    (let [rst' (reduce (fn [acc _] (dfs acc)) rst (range ch))]
;      (prn (take met rst'))
      (swap! *result + (apply + (take met rst')))
      (drop met rst'))))
;
;(dfs (parse test-input))
;
;(dfs (parse input))

@*result


;; 2 part

(defn dfs [input]
  (let [[ch met & rst] input]
    (if (= ch 0)
      {:rest (drop met rst)
       :meta (apply + (take met rst))}
      (let [{rst' :rest
             metas :metas}
            (reduce (fn [acc _]
                      (let [{:keys [rest meta]} (dfs (:rest acc))]
                        {:rest rest :metas (conj (:metas acc) meta)}))
                    {:rest rst :metas []}
                    (range ch))]
        {:rest (drop met rst')
         :meta (apply + (keep (fn [idx] (get metas (dec idx))) (take met rst')))}))))

(dfs (parse test-input))
(dfs (parse input))