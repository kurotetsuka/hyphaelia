# Operation modes of the dttp daemon
Any subset of these operation modes may be enabled at either initialization or run time. Some modes do depend on the data produced from others, so not every possible combination may be practical. For example, in order to pull a desired mote, the daemon must have some tracking information. It would, however, be possible to enable tracking just long enough to find the desired mote, then disable it. These choices are the responsibility of the administrator.

todo:  
 - add sector handling stuff

## Metadata modes
These modes all perform various operations intended to gather data about motes or hubs.

### Tracking
keeping track of who has certain motes, and who knows who has certain motes, etc.

### Bootstrapping
discovering other hubs, keeping track of them, detecting when hubs leave the network, etc.  
assisting other's hub discovery by serving above information.

### Profiling
benchmarking other hubs  
 - connection speed
 - distance
 - tracking ability
 - storage ability
 - reliability
 - integrity

## Data modes
These modes all perform operations on motes.

### Pollination
Assisting initial distribution process  
 - informing trackers of new mote
 - seeding : pushing to initial hub(s)
 - dispersion : hubs pushing to other hubs

### Pushing
continually asking other hubs to pull our new data

### Pulling
grabbing desired motes off of other hubs.

### Serving
responding to pulls

### Onion serving
acting as a step in onion routing of pulls

## Analysis modes
These modes all perform various types of analysis on gathered motes.

### Curation
Hubs may wish to remove junk, spam, or old motes from their servers. Additionally, some hubs may need to respond to legal takedown requests. Thus, automatic parameter-based curation may be desired. While this could theoretically enable censorship, the distributed nature of the system makes such censorship an optional, democrative process.

### Thread reconstruction
asdf

### Trust analysis
asdf

### Respect analysis
asdf

## Client serving modes
These modes are all solely intended to serve data to end-users. They are not expected to be useful to other hubs, since any hub could ( and should ) perform this themselves.

### Thread ranking
Ranking of mote-threads according to user's circles and respect rankings. This requires some knowledge of the user, and thus a prior registration with the daemon.

### Thread serving
Compilation of entire mote-threads for user's perusal. This requires some thread reconstruction.

### Encrypted mote serving
Users may wish to distribute a mote only to certain other users, and only them. There are two ways to do this - encrypted mote serving is one and private hubs are the other. The daemon stores the unencrypted mote, and only serves copies that are encrypted for authorized target(s). This requires the source user to implicitly trust the serving hub. Once encrypted, motes may be served and distributed normally
