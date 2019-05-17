Changelog
=========================

We have two kinds of updates. A major update occurs when y changes in x.y.z. A minor update when z changes.

Major updates can introduce code incompabilities with code that uses a previous version of this library. Minor updates are garantueed to only include bug fixes or upgrades of a few depedencies.

## Version 0.4.1

- Bump Rocket to 0.4.1

## Version 0.4

- Upgrade to Rocket 0.4.
- This is also a nearly complete rewrite of the [rocket-simpleauth](https://github.com/bramvdbogaerde/auth-rs) crate, which took inspiration of the former work of Bram Vandenbogaerde. Why was this done? Because I wanted to create a canonical authentication mechanism for [Rocket](https://github.com/SergioBenitez/Rocket), or at least start at some point, that...
1. is easy to use
2. is also flexible enough
3. gets the "Approved by Sergio"-stamp


## Version 0.2

Removed the FromCookie trait. We use Rocket's private cookie system by default. So LoginStatus doesn't need a type parameter for FromCookie anymore, see example/ for more information.

## Version 0.1.1

Upgraded to Rocket 0.2

## Version 0.1.0

Introduction of the library

