#lang racket

(define (run f)
  (call-with-input-file f find-marker))

(define (find-marker in)
  (let*
   ([input (string->list (port->string in))]
    [packet-mk (count-until-unique input 4)]
    [input (drop input packet-mk)]
    [msg-mk (count-until-unique input 14)])
   (+ packet-mk msg-mk)))

(define (count-until-unique xs n-pref)
  (define (count-until-acc xs n)
    (cond [(empty? xs) 'undefined]
          [(unique-n-prefix? xs n-pref) n]
          [else (count-until-acc (rest xs) (add1 n))]))
  (+ n-pref (count-until-acc xs 0)))

(define (unique-n-prefix? xs n)
  (if (<= n (length xs)) (unique? (take xs n)) #f))

(define (unique? xs)
  (not (check-duplicates xs)))
