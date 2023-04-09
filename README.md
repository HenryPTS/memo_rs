# SuperMemoRS

implementation of supermemo2 for rust

## todo
- support for other versions of supermemo
- support for forks of SM2 such as the one used by anki

## usage
```rust

fn incorrect_response_first_try() {
  let res = SuperMemo2::new().calc(0, 0, 2.5, 0);
  assert_eq!(res, (0, 0, 2.5, 1));
}

fn grade_over_max_length() {
  super_memo2(50, 2, 1.3, 6);
}
```