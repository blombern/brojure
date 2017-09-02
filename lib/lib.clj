(do

  (defn not [x]
    (if x false true))

  (defn pos? [x]
    (> x 0))

  (defn neg? [x]
    (if (= x 0) false (< x 0)))

  (defn odd? [x]
    (if (= (mod x 2) 1) true false))

  (defn even? [x]
    (if (= (mod x 2) 0) true false))

  (defn inc [x]
    (+ x 1))

  (defn dec [x]
    (- x 1))

  (defn sum [v]
    (reduce (fn [acc x] (+ acc x)) 0 v))

  (defn product [v]
    (reduce (fn [acc x] (* acc x)) 1 v))

  (defn pop [v]
    (for [i (range 0 (dec (count v)))]
      (inc i)))

  (defn take [n v]
    (for [i (range 0 n)]
      (nth v i)))

  (defn drop [n v]
    (for [i (range n (count v))]
      (nth v i)))

  (defn map [f v]
    (for [i v]
      (f i)))

  (defn filter [f v]
    (for [i v]
      (if (f i)
        i)))

  (defn euler1 []
    (let [multiple-of-3-or-5?
          (fn [n]
            (or (= (mod n 3) 0)
                (= (mod n 5) 0)))]

      (sum (filter multiple-of-3-or-5? (range 1 1000)))))

)
