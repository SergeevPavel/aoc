(ns code.fourteen)

(def initial-state {:v [3 7]
                    :f 0
                    :s 1})

(defn step [{:keys [v f s]}]
  (let [new-v (into v (map #(- (int %) (int \0)) (str (+ (nth v f) (nth v s)))))
        new-f (mod (+ f (inc (nth v f))) (count new-v))
        new-s (mod (+ s (inc (nth v s))) (count new-v))]
    {:v new-v
     :f new-f
     :s new-s}))

(defn ->str [{:keys [v f s]}]
  (apply str
         (interpose " "
                    (map-indexed (fn [idx val]
                                   (cond
                                     (= idx f) (str "(" val ")")
                                     (= idx s) (str "[" val "]")
                                     :else     val))
                                 v))))


(defn solve [input]
  (loop [st initial-state]
    (if (< (+ input 10) (count (:v st)))
      (apply str (take 10 (drop input (:v st))))
      (recur (step st)))))

(defn solve1 [input-number]
  (let [input (into [] (map #(- (int %) (int \0)) (str input-number)))]
    (loop [st initial-state]
      (let [v (:v st)
            end (count v)
            start (max 0 (- end (count input)))

            start1 (max 0 (dec start))
            end1 (max 0 (dec end))]
        (if (= (subvec v start end) input)
          start
          (if (= (subvec v start1 end1) input)
            start1
            (recur (step st))))))))

(solve1 51589)
(solve1 01245)
(solve1 92510)
(solve1 59414)

(solve1 919901)

;(doseq [s (take 16 (iterate step initial-state))]
;  (prn (->str s)))

