# Requirements

Senior Software Engineer Take-Home Programming Assignment for Rust

## Story

There are over 100,000 flights a day, with millions of people and cargo being transferred around the world. With so many people, and different carrier/agency groups it can be hard to track where a person might be. In order to determine the flight path of a person, we must sort through all of their flight records.

## Goal

To create a microservice API that can help us understand and track how a particular person’s flight path may be queried. The API should accept a request that includes a list of flights, which are defined by a source and destination airport code. These flights may not be listed in order and will need to be sorted to find the total flight paths starting and ending airports.

## Required structure:

[['SFO', 'EWR']] => ['SFO', 'EWR']
[['ATL', 'EWR'], ['SFO', 'ATL']] => ['SFO', 'EWR']
[['IND', 'EWR'], ['SFO', 'ATL'], ['GSO', 'IND'], ['ATL', 'GSO']] => ['SFO', 'EWR']

# Solution

## Usage

You can input the required structure inside quotation marks as follows.

`$./test.sh "[['IND', 'EWR'], ['SFO', 'ATL'], ['GSO', 'IND'], ['ATL', 'GSO']]"`

## Algorithms

1. Find the starting and ending airports. Count the frequency of each airport. If the frequency is an odd number, that means the corresponding airports are candidates for starting and ending airports. If there are more than 2 airports of which frequencies are odd numbers, it means that the input is incorrect.
2. Distinguish between starting and ending airports.
3. Follow the route from start to end to check if the input is valid one.
