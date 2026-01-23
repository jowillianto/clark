# Features
There are 4 active features in the current repo: 
- log: logging utilities
- app: application argument parsing utilities
- tui: terminal ui building dom abstractions
- async: asynchronous trait for ActionBuilder
- py: python bindings

# Python Bindings Conventions
When writing python bindings and needing to forward functions or create structures, do not rewrite structures or functions but instead do the following as an example: 
```rs
struct FizzBuzz {
}

impl FizzBuzz {
  fn fizz(&self);
  fn buzz(&self);
}

mod py {
  #[pyo3::pyclass]
  struct FizzBuzz {
    inner: super::FizzBuzz
  }
  
  impl From<super::FizzBuzz> for FizzBuzz {
      fn from(inner: super::FizzBuzz) -> Self {
          Self { inner }
      }
  }
  
  #[pyo3::pymethods]
  impl FizzBuzz {
    fn fizz(&self) -> {
        self.inner.fizz()
    }
    fn buzz(&self) -> {
        self.inner.buzz()
    }
  }
}
```
Everything should be bound manually into the class. 