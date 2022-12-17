#lang racket

(define (run fname)
  (call-with-input-file fname supply-stack))

(define (supply-stack in)
  (define-values (stacks-input moves-input) (split-input in))
  (define stacks (make-stacks stacks-input))
  (define moves (make-moves moves-input))
  (define end-stacks (play-moves stacks moves))
  (list->string (map car end-stacks)))
;; A Stack is a list

;; A move has a:
;; - `num' : number of boxes to move
;; - `from': the index of a stack to pop boxes from
;; - `to'  : the index of the stack to push boxes onto
(struct move (num from to))

(define (split-input in)
  (define-values (stack-in move-in _)
    (for/fold ([s-in '()] [m-in '()] [moves? #f]) ([line (in-lines in)])
      (cond
        [(= (string-length line) 0) (values s-in m-in #t)]
        [moves? (values s-in (cons line m-in) moves?)]
        [else (values (cons line s-in) m-in moves?)])))
  ; we don't reverse the stack list because its easier to construct
  ; the stack input bottom up
  (values stack-in (reverse move-in)))

(define no-crate "   ")

(define (make-stacks stacks-input)
  (define init-stack (lambda (_) '()))
  (define init-stacks (map init-stack (string-split (first stacks-input))))
  (for/fold ([sts init-stacks]) ([crates (rest stacks-input)])
    (add-crates crates sts)))

(define (add-crates crates sts)
  (define split-crates (crates->list crates))
  (for/list ([stack sts] [crate split-crates])
    (if (has-crate? crate) (cons (string->crate crate) stack) stack)))

(define (crates->list crates)
  (define crate-rx #rx"    |.[A-Z].|   ")
  (regexp-match* crate-rx crates))

(define (string->crate cs)
  (second (string->list cs)))

(define (has-crate? crate)
  (regexp-match #rx".[A-Z]." crate))

(define (make-moves moves-input)
  (map string->move moves-input))

(define (string->move movestr)
  (define (to-idx s)
    (sub1 (string->number s)))
  (match (string-split movestr)
    [(list "move" n "from" f "to" t) (move (string->number n) (to-idx f) (to-idx t))]
    [_ (move 0 0 0)]))

(define (play-moves stacks moves)
  (foldl play-move stacks moves))

#;(define (play-move m sts)
    (match m
      [(move 0 from to) sts]
      [(move num from to) (play-move (move (sub1 num) from to) (move-crate sts from to))]))
(define (play-move m sts)
  (let* ([n (move-num m)]
         [f (move-from m)]
         [t (move-to m)]
         [from (list-ref sts f)]
         [to (list-ref sts t)]
         [top-n (take from n)]
         [new-from (drop from n)]
         [new-to (append top-n to)])
    (list-set (list-set sts f new-from) t new-to)))

(define (move-crate sts f t)
  (let* ([from (list-ref sts f)]
         [to (list-ref sts t)]
         [crate (first from)]
         [n-from (rest from)]
         [n-to (cons crate to)])
    (list-set (list-set sts f n-from) t n-to)))
