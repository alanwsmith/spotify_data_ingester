# Spotify Data Ingester

For pulling Spotify data into a SQLite database
for further analysis. 

The data comes from Spotify via their
[account data download page](https://www.spotify.com/us/account/privacy/)

The current version of the tool works with
the latest year data package. Once I get
my hands on my "Extended streaming history"
data, I'll update for that if any changes
are necessary. 

## Data Points

The data that's pulled in is:

- StreamingHistory#.json files that go to 
the streaming_history table. 


## Notes

- For the library, I don't have shows, episodes, 
bannedTracks, or bannedArtists. I'm not sure what
the format of those is. I haven't found documentation
on those data structures which means the process
will most likely break if that data shows up. Open
an issue with sample data if you see that and I'll put 
them in place. 
