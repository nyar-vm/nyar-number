(* ::Package:: *)

(* ::Title:: *)
(*Truncatable Prime*)


(* ::Subsection:: *)
(*Author*)


(* ::Text:: *)
(*Eric W. Weisstein*)
(*November 4, 2003*)


(* ::Text:: *)
(*This notebook downloaded from http://mathworld.wolfram.com/notebooks/PrimeNumbers/TruncatablePrime.nb.*)


(* ::Text:: *)
(*For more information, see Eric's MathWorld entry http://mathworld.wolfram.com/TruncatablePrime.html.*)


(* ::Text:: *)
(*\[Copyright]2005 Wolfram Research, Inc. except for portions noted otherwise*)


(* ::Subsection:: *)
(*Author*)


(* ::Text:: *)
(*Aster*)
(*July 20, 2019*)


(* ::Text:: *)
(*Improve the algorithm*)


(* ::Subsection:: *)
(*Code*)


LeftTruncatablePrimeQ::usage = "LeftTruncatablePrimeQ[n] returns True if n and every substring of digits from the left is a prime number."

LeftTruncatablePrimes::usage = "LeftTruncatablePrimes[n] gives the n-digit left prime strings."

LeftTruncatablePrimesRestricted::usage = "LeftTruncatablePrimesRestricted[n] gives a list of the left truncatable (Henry VIII) primes of length n."

RightTruncatablePrimeQ::usage = "RightTruncatablePrimeQ[n] returns True if n and every substring of digits from the right is a prime number."

RightTruncatablePrimes::usage = "RightTruncatablePrime[n] gives the n-digit right prime strings."


SetDirectory@NotebookDirectory[];

(*Left Prime Strings*)

LeftTruncatablePrimes[1] = Select[Range@9, PrimeQ];
LeftTruncatablePrimes[n_] := LeftTruncatablePrimes[n] = Select[#1 * 10^(n - 1) + #2& @@@ Tuples[{Range@9, LeftTruncatablePrimes[n - 1]}], PrimeQ];

LeftTruncatablePrimesRestricted[1] := {}

LeftTruncatablePrimesRestricted[n_] := LeftTruncatablePrimesRestricted[n] = Select[LeftTruncatablePrimes[n], !Function[f, MemberQ[LeftTruncatablePrimes[n + 1], f]] /@ Or @@ prependedDigitList[#]&]

prependedDigitList[n_] := FromDigits[Prepend[IntegerDigits[n], #]]& /@ Range[9]

(*Right Prime Strings*)

RightTruncatablePrimes[1] = Select[Range@9, PrimeQ];
RightTruncatablePrimes[n_] := RightTruncatablePrimes[n] = Select[10 * #1 + #2 & @@@ Tuples[{RightTruncatablePrimes[n - 1], {1, 3, 6, 7, 8, 9}}], PrimeQ];


(* ::Section:: *)
(*Left Truncatable Primes*)


Module[
	{LTPs},
	LTPs = LeftTruncatablePrimes /@ Range[25] // Flatten;
	Export["truncatable_primes_left.txt", StringRiffle[LTPs, "\n"], "Plaintext", CharacterEncoding -> "UTF8"]
]


(* ::Subsection:: *)
(*Restricted*)


Length /@ (lr = LeftTruncatablePrimesRestricted /@ Range[25])


Plus @@%


Take[Flatten[lr], 20]


{#, IntegerDigits[#] // Length}&[Last[Flatten[lr]]]


(* ::Section:: *)
(*Right Truncatable Primes*)


Module[
	{RTPs},
	RTPs = RightTruncatablePrimes /@ Range[10] // Flatten;
	Export["truncatable_primes_right.txt", StringRiffle[LTPs, "\n"], "Plaintext", CharacterEncoding -> "UTF8"]
]


(* ::Subsection:: *)
(*Odd*)


Select[Flatten[r], And @@ (OddQ /@ IntegerDigits[#])&]


RiveraRightPrimes = {3, 7, 13, 17, 37, 73, 97, 113, 137, 173, 197,
	313, 317, 337, 373, 397, 773, 797, 937, 997, 1373, 1997, 3137, 3313,
	3373, 3797, 7937, 9137, 9173, 9337, 9397, 13313, 33797, 39397, 79337,
	79397, 91373, 91997, 99137, 99173, 99397, 139397, 379397,
	391373, 399137, 399173, 739397, 933797, 979337, 3399173,
	3739397, 9139397, 9391373, 9979337, 33739397, 39979337,
	99979337, 933739397};
