# Assayer

### a•SEY•er, n.
1. A detailed report of the findings in analysis of a substance
2. An unfortunate person whose job it was to taste test royal food, in case of poison
3. Static (compile-time) validation library for Rust

## Motivation
Frequently, programs need to process untrusted input.  Before doing so, it is important to ensure that the values being
processed are, in fact, valid for the purpose for which they are intended.

Values representing the hour of the day, the day of the month, a person's age, a URL, phone number, and so on.
Regardless of the underlying data type (integer, string, custom struct, etc.) some values are valid, and some are not.

As a validation library, Assayer allows you to build efficient, highly cohesive code by concentrating your custom
validation logic to one place--your validator.  You can create any number of validators for each of the different types
in your application requiring validation.

## How to Use
### Quickstart  
- Define your custom validator (eg. for non-empty strings) by implementing `ValidatorRef` and/or `ValidatorMutRef`
and/or `Validator`.  `ValidatorRef` example:

```rust
impl assayer::ValidatorRef<String> for NonEmptyStringValidator {
    fn validate_ref(v: &String) -> assayer::Result<&String> {
       Ok(v)
           .and_then(|v| match !v.is_empty() {
                true => Ok(v),
                false => Err(assayer::Error::ValueEmpty("Empty Value Error".to_string())),
            })
         
           // Chain as many additional .and_then() clauses as you need to validate your input.
           // Each validation clause in the .and_then() chain must return:
           //   * Ok(v) if the value passes that validation step or
           //   * Err(String) if the value fails that validation step.
           // A clause in a Result::and_then() chain which returns Err() will terminate further validation processing
           // and the returned Err() will be returned to the caller.
           
           //.and_then(|v| ...test #2...)
           //...
           //.and_then(|v| ...test #n...)
    }
}
```
To call the the validator, use either the function- or method-call syntax illustrated below:

```rust
let foo = String::new();    //This is an empty String
let bar = "I am not empty".to_string();

// Function synthax
let function_call_syntax_result = String::validate_ref::<NonEmptyStringValidator>(&foo);

// Method synthax
let method_call_syntax_result = (&bar).validate_ref::<NonEmptyStringValidator>();

assert_eq!(function_call_syntax_result.err(), Some(assayer::Error::ValueEmpty("Empty Value Error".to_string())));
assert_eq!(method_call_syntax_result.ok(), Some(&bar));
```
### FAQ
#### What are some of Assayer's design goals?
##### • Static (compile-time) validator
As a static validator, the validation chain you create is able to fully benefit from the compiler optimizers.  A dynamic
validator using, for example the builder pattern at run-time cannot realize these benefits, because the validation chain
does not exist as compile time.

We thought about use-cases, and other than the case of deciding how to validate at run-time (not the intended purpose of
this library), we could not think of a drawback for static validators.  If we come across one, we are open to augmenting
this implementation with a run-time variant.

##### • Lightweight, Zero Dependencies
Languages have long been heading down the path of pulling in other "modules" and building on top of them (the Javascript
community is particularly notable in this regard).  Rust's Cargo and crate system brings easy dependencies to Rust,
which we fully welcome.  At the same time, we are fans of beautiful, maintainable, predictable, small focuse code.  When
less code can do more, we think that kind of efficiency gain is particularly elegant.  So the library ended up being a
grand total of six trait definitions, plus three blanket impls (which is how `MethodSyntaxValidator<SomeType>` gets defined
automatically when you as the user implement only `Validator<SomeType>`), and no dependencies.
you as the user of the library )

##### • Facilitating High Cohesion, Single-Responsiblity, Don't Repeat Yourself Principles
A validator allows you to factor out all your checking into pre-conditions all in one place.  When all the code in a
routine is focused on achieving the same goal, you get high cohesion, which as been shown to be very beneficial to the
reliability and maintainability of a given codebase.

By focusing only validation code into a validator, the validator becomes responsible for just one thing: validation.
The single-responsibility principle helps to keep the code from growing into a "god object", which, if you've ever had
the pleasure of working on one, we're sure you'll agree you don't want to go back any time soon!

#### What is the difference between function-call syntax and method-call syntax?
From a code perspective, none.  (For the tests we've run under Rust v1.18.0, the compiler yields 100% identical assembly
instructions.)  From a personal perspective, you may be more comfortable with one style or the other, or it may be
convenient to switch styles under different circumstances.  The choice is yours.

#### When should I implement `Validator` vs. `ValidatorRef` vs. `ValidatorMutRef`?
Implementing `Validator` means that the call will consume your value:

```rust
impl assayer::Validator<String> for NonEmptyStringValidator {
    fn validate(v: String) -> assayer::Result<String> {
       Ok(v)
           .and_then(|v| match !v.is_empty() {
                true => Ok(v),
                false => Err(assayer::Error::ValueEmpty("Empty Value Error".to_string())),
            })
    }
}

fn main() {
    let input = "Not an empty string".to_string();
    let result = input.validate::<NonEmptyStringValidator>();
    // assert_eq!(result.ok(), Some(input)); //Oops!  input has been moved!
    assert_eq!(result.ok(), Some("Not an empty string".to_string()));
}
```
To remedy this, either use `input.clone().validate::<NonEmptyStringValidator>()` or implement `ValidateRef` and use
`(&input).validate_ref::<NonEmptyStringValidator>()`.

The case for implementing `ValidatorMutRef` is somewhat rare and arguably a violation of the Single Responsibility
Principle.  Whether you consider it to be or not, it can be used to 'fix up' an incoming value before it is used by your
post-validation code (you might want to pad or round the input value before it is used, for example).  Here is how it is used:

```rust
impl assayer::ValidatorMutRef<String> for NonEmptyStringValidator {
    fn validate_mut_ref(v: &mut String) -> assayer::Result<&mut String> {
       Ok(v)
           .and_then(|v| match !v.is_empty() {
                true => { v.push_str("!"); Ok(v) }, 
                false => Err(assayer::Error::ValueEmpty("Empty Value Error".to_string())),
            })
    }
}

fn main() {
    let mut input = "Not an empty string".to_string();
    let mut expected_result_inner = "Not an empty string!".to_string();
    let expected_result = Some(&mut expected_result_inner);
    
    let result = (&mut input).validate_mut_ref::<NonEmptyStringValidator>();
    assert_eq!(result.ok(), expected_result);
}
```

Finally, if you simply want all three implementations to "just work" whenver you implement one, implement ValidateRef
as per your needs, and ValidateMutRef and Validate as follows:

TODO: Provide blanket impls for ValidateMutRef & Validate, in terms of ValidateRef.