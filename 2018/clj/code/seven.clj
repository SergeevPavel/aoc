(ns code.seven
  (:require [clojure.set :as set]))

;
(def input (line-seq (clojure.java.io/reader "/Users/pavel/hobby/adventofcode/code/resources/day7.txt")))

(def input (clojure.string/split-lines "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."))

(type input)

(defn parse [s]
  (let [[_ x y] (re-matches #"Step (.) must be finished before step (.) can begin." s)]
    [(keyword x) (keyword y)]))

(def g (reduce (fn [g [y x]]
                 (-> g
                     (update y #(or % #{}))
                     (update x (fn [scs]
                                 (conj (or scs #{}) y)))))
               {}
               (map parse input)))

(defn step [g]
  (let [n (->> g
               (filter (fn [[k v]] (empty? v)))
               (sort-by first)
               (first)
               (first))]
    [(->> g
          (dissoc g n)
          (map (fn [[k v]] [k (disj v n)]))
          (into {}))
     n]))


(apply str (map name (loop [g g ans []]
                       (if-not (empty? g)
                         (let [[g' n] (step g)]
                           (recur g' (conj ans n)))
                         ans))))

(def workers-count 5)

(defn select-task [g]
  (->> g
       (filter (fn [[k v]] (empty? v)))
       (sort-by first)
       (first)
       (first)))

(defn task->time [tsk]
  (+ 61 (- (int (first (name tsk))) (int \A))))

(disj #{} nil)

(defn solve []
  (loop [g g
         workers (into {} (map-indexed vector
                                       (repeat
                                        workers-count
                                        {:task nil
                                         :alarm 0})))
         t 0]
    (let [alarmed-workers (filter (fn [[id {:keys [alarm]}]] (= alarm t)) workers)
          g' (reduce (fn [acc tsk]
                       (when (some? tsk)
                         (prn tsk " " t))
                       (->> acc
                            (map (fn [[k v]] [k (disj v tsk)]))
                            (into {})))
                     g
                     (map (fn [[id {:keys [task]}]] task) alarmed-workers))
          {g'' :g
           workers' :w} (reduce
                         (fn [{:keys [g w]} id]
                           (if-let [tsk (select-task g)]
                             {:g (dissoc g tsk)
                              :w (assoc w id {:task tsk
                                              :alarm (+ t (task->time tsk))})}

                             {:g g
                              :w (assoc w id {:task nil
                                              :alarm (inc t)})}))
                         {:g g' :w workers}
                         (map first alarmed-workers))]
      (if-not (and (empty? (filter (fn [w] (:task (second w))) workers')) (empty? g''))
        (recur g'' workers' (inc t))
        t))))


(solve)
