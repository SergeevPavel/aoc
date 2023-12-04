(ns code.six
  (:require [clojure.string :refer [split trim]]
            [clojure.set :as set]))
;
;(def cs [[1 1]
;         [1 6]
;         [8 3]
;         [3 4]
;         [5 5]
;         [8 9]])

(def input (line-seq (clojure.java.io/reader "/Users/pavel/hobby/adventofcode/code/resources/day6.txt")))

(def cs (map (fn [s]
               (mapv (fn [n] (Long/parseLong (trim n))) (split s #","))) input))


(def min-x (apply min (map first cs)))
(def max-x (apply max (map first cs)))

(def min-y (apply min (map second cs)))
(def max-y (apply max (map second cs)))

(- max-x min-x)
(- max-y min-y)

;;

(def iteration-count 700)

(defn step-point [[x y]]
  #{[(dec x) y]
    [(inc x) y]
    [x (dec y)]
    [x (inc y)]})

(defn step [{:keys [front body]}]
  (set/difference (into #{} (mapcat step-point front))
                  body))

(def initial-state {:ters (map (fn [[x y :as cc]] {:front #{cc} :body #{cc}}) cs)
                    :moot #{}})

(defn inflate [{moot :moot state :ters}]
  (let [fronts' (mapv step state)
        moot' (apply set/union moot
                     (map (fn [i]
                            (let [f (get fronts' i)]
                              (set/intersection f (apply set/union (assoc fronts' i nil)))))
                          (range (count fronts'))))
        busy (apply set/union (map :body state))]
    {:moot moot'
     :ters (map (fn [s f]
                  (let [f' (set/difference f moot' busy)]
                    {:front f' :body (set/union (:body s) f')}))
                state
                fronts')}))

(def state (:ters initial-state))
(inflate initial-state)
;
(def res (take 200 (iterate inflate initial-state)))
;(+ 22 2)
;
(def one (:ters (first (reverse res))))
(:moot (first (reverse res)))
;(def two (:ters (second (reverse res))))
;
;(count (filter #(empty? (:front %)) two))
;(apply max (map #(count (:body %)) (filter #(empty? (:front %)) two)))
;;
;
(count (filter #(empty? (:front %)) one))
(apply max (map #(count (:body %)) (filter #(empty? (:front %)) one)))


;; 6.2


(def mdist 10000)
(/ mdist (count cs))

(count cs)

(* 800 800)

(->> (for [i (range -200 580)
           j (range -200 580)]
       (apply + (map (fn [[x y]]
                       (+ (Math/abs (- x i)) (Math/abs (- y j))))
                cs)))
     (filter #(< % 10000))
     (count))
