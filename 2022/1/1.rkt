#lang racket

;; Runs `max-calories' with the data in `filename'
(define (run filename)
  (call-with-input-file filename top-three-calories))

;; Finds the elf with the most calories in their meals
;; calculates calories from `in'
;; Port -> Number
(define (max-calories in)
  (top-n in 1))

(define (top-three-calories in)
  (top-n in 3))

;; Finds the elf with the most calories in their meals
;; calculates calories from `in'
;; Port -> Number
(define (top-n in n)
  (foldr + 0 (take (sort (total-calories in) >) n)))

;; reads a file line by line, summing sequences of numbers separated by blank lines.
;; assumes that `in' is a file that only contains Numbers and newlines
;;
;; returns a list of the total calorie count of every elf
(define (total-calories in)
  (define (process-line line totals sum)
    (if (non-empty-string? line)
        (values totals (+ sum (string->number line)))
        (values (cons sum totals) 0)))
  (define-values (totals _)
    (for/fold ([totals '()] [sum 0]) ([line (in-lines in)])
      (process-line line totals sum)))
  totals)
