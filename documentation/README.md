# Data model of this crate

This crate aims to extract data from OSM with as less transformation as possible, but make some changes in properties' name to ease usability.

## Objects description
The objects extracted from OSM are the following :
* Line : corresponding to OSM relation (type=route_master)[https://wiki.openstreetmap.org/wiki/FR:Relation:route_master]
* Route : corresponding to OSM relation (type=route)[https://wiki.openstreetmap.org/wiki/FR:Relation:route]
* StopPosition : Place where the bus/tram/metro/... stops
* Platform : Place where passengers are waiting to embark in a bus/tram/metro/...
* StopArea : Group of StopPositions and Platforms generaly known with a single name (like a train station for exemple)

## Extraction of OSM Data
Public tranport data in OSM is very complex, with several schemas possible to describe a same transport object.

The method used to extract data is the following:
* Extracting all the Stops (either StopPositions or Platforms) only for bus as a first step. A bus stop is defined by a way or a node containing either:
  * `highway=bus_stop`
  * or `public_transport=platform` or `public_transport=stop_position` also having either `highway=bus_stop` or `bus=yes`
* Extracting all the StopAreas : relations with `public_transport=stop_area`
* Extracting all the Public Transport Routes and Lines (see below)
* Defining for each Stop if it's a StopPosition or a Platform (see below)


**Extraction of Public Transport Routes and Lines**
To define if a relation is a public transport Route (resp. Line), the following method is used :
* The relation contains the tag `type=route` (resp. `type=route_master`)
* The relation contains the tag `route` with a value contained in the followings:
  * trolleybus
  * bus
  * train
  * subway
  * light_rail
  * monorail
  * tram
  * railway
  * ferry
  * coach
  * aerialway
  * funicular
  * rail
  * share_taxi

A complementary extraction is made with potential PT relations that has a `route` (resp. `route_master`) tag value that is **not** contained in the list:
* bicycle
* canoe
* detour
* fitness_trail
* foot
* hiking
* horse
* inline_skates
* mtb
* nordic_walking
* pipeline
* piste
* power
* proposed
* road
* running
* ski
* historic
* path
* junction
* tracks



**Categorization of Stops**

* If the tag `public_transport` is set
  * with `stop_position` => the object is a StopPosition
  * with `platform` => the object is a Platform
  * else : consider value as invalid and continue as if `public_transport` is unset (see below)
* else if the property `highway=bus_stop` is present and the object is contained in (at least) a Route
  * if one of the Routes have `public_transport:version = 2`
    * if the stop has the role `platform`, `platform_exit_only`, `platform_entry_only` => the object is a Platform
    * if the stop has the role `stop`, `stop_exit_only` or `stop_entry_only` => the object is a StopPosition
    * else => check with an other Route
  * else => this stop is unknown

Unknown stops are considered to be Plaforms for the extraction.
