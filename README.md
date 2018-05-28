# rustyearth

A simple space api implementation for the AfRA.

This application can be run directly or behind a https/http reverse proxy (e.g. nginx).

#  http:// endpoints

## GET
/ - (index) - TODO: still empty. Should have a howto use
/v1/status - returns the open/close status as Text. E.g: "Status true"
/v1/status.json - contains the space api in json format. (MaxAge 1 min)
/v1/status.png - returns an open or closed picture. (MaxAge 1 min)

## POST

/v1/status/TOKEN/0 - closes the space
/v1/status/TOKEN/1 - opens the space

The TOKEN is a secret which have to be defined at compiling time.
The default is choosen by a random dice Hee2noh8aic3iech
