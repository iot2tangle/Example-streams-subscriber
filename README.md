# Example Streams Subscriber

## Introdution
This is an example implementation of a real-time streams decoder.  
When started it will read and print all existing messages in a channel, and afterwards start to listen for new ones.
  
## Usage
To download the Streams Subscriber run:  
`git clone https://github.com/iot2tangle/example-streams-subscriber`  
`cd example-streams-subscriber`  
  
<br>  

To subscribe to a channel run:  
`cargo run --release <channel_address>`  
And put the address of the channel you want to subscribe into "<channel_address>"


## Example
To run an example:  
`cargo run --release 64e592476ed7619d04526f6360f150d4bce63046511dde3f8665e5aec6b51ffc0000000000000000:78829daa792010edd2c7dbfb`  
This will read out all the messages for that channel!

## NOTE  
We have noticed that if there are too many messages(10/15 +) in the channel the subscriber will get stuck sometimes!

