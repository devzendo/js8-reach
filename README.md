# js8-reach

An experiment to show the reachability of JS8Call stations. 

From your station, you might hear some other stations, and they might hear you. If you hear a station and they hear you, there is two-way reachability. This is also the case for all those stations. The reachability forms many chains between your station, and distant ones. (Mathematically, it's a graph.)

How far can you reach? Can you reach your distant friend? Are there paths between you and other stations that are sufficiently reliable to allow relayed communications?

This experiment takes a feed of JS8Call data from PSKReporter's MQTT feed, and given your callsign (and any stations you want to establish a relayed call with), displays the graph of reachability on a map.

Available as either a locally-runnable application, for a single station, or as a cloud service that can be used by many stations (each of which require a login).



Acknowledgements

Rocket server, Yew front end framework.

App set up using guide at https://theadventuresofaliceandbob.com/posts/rust_rocket_yew_part1.md

