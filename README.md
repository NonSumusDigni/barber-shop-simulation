# barber-shop-simulation

A small program which simulates the operation of a barber shop.

## Running Instructions

If you do not already have Rust installed, you can install it by following the
instructions [here](https://www.rust-lang.org/tools/install).

Then, in the root directory of the project, run the following command:

```bash
cargo run
```

## Implementation

Thinking through the problem, I settled on an approach with two main components - a state object
representing the barber shop, and a queue of events which apply mutations to the state (and often
generate further events). My initial approach had these two concepts combined into a larger
structure, but as I went down that road I found that satisfying Rust's ownership rules was
becoming painful, so I pulled them apart.

Most of the implementation went smoothly, but there were a number of limitations to my design I
ran into as the final pieces came together:

- The SortedVec I had chosen for the event queue, while convenient for ease of insertion and
  removal based on timestamp order, did not behave as I had expected regarding tie-breaking. As
  I did not have time to swap out the data structure or implement my own, I solved this with a
  hack, where I wrapped the EventEnvelopes in another structure that contained a monotonically
  increasing counter, which was used as a tie-breaker. This was not ideal, but it achieved the
  behavior I was relying on, with tie-breaking resolving to insertion order.
- The separation of state and events ended up less clean than I had hoped, as during the
  application of an event, we often needed some details from the state to determine what logs
  and new events to emit. This is especially apparent in the haircut event, where the
  haircut_complete function returns a massive tuple.
- As completion came near and I was mindful of the clock, I ended up copying some pieces of code
  to multiple spots to make everything work. This was in places with lots of branching logic on
  event application, such as a haircut completing (barber might end shift, customer might be in
  line, after barber shift shop might close, etc).

I think the general structure made sense, but there are improvements I would make if I spent
more time:

- I would implement my own data structure for managing the event queue, which would tie-break by
  insertion order natively instead of wrapping the envelope.
- Having the generation of subsequent events come from within the event `apply` functions is
  probably a mistake. The state functions themselves should output events instead, so that the
  function signatures can make more sense in their own right, rather than arbitrarily returning
  various pieces of the state due to coupling with the event handlers.
- There are lots of static values that could be configurations or command line parameters, of
  course, as well as panics instead of proper error handling, but I think that comes with the
  territory on a time-limited project.

I didn't end up having time for the bonus prompt, with input files and unit tests. Admittedly, I
ended up running over time a fair bit, I spent about 3.5 hours. Picking Rust was perhaps a
mistake here, as nontrivial time was spent battling with the borrow checker, and the structure
was informed by avoiding ownership issues. Things would have been a lot easier in a language
like Python. But Rust is my day-to-day and most familiar language at the moment, so that was
what I selected.