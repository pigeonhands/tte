## Basic usage
`tte.exe ping -s 1 8.8.8.8`
```
Executed in 3.022309600s
```

## Extended Output
`tte -e ping -s 1 8.8.8.8`

```
ping -s 1 8.8.8.8

Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 3
Milliseconds      : 0
Nanoseconds       : 3021453800

Total Days        : 0.0000349705300925926709265800
Total Hours       : 0.0008392927222222222893656400
Total Minutes     : 0.0503575633333333334340484600
Total Seconds     : 3.021453800
Total Milliseconds: 3021453.800
```


## Output with STDOUT
`tte -o ping -s 1 8.8.8.8`

```
Pinging 8.8.8.8 with 32 bytes of data:
Reply from 8.8.8.8: bytes=32 time=5ms TTL=254
Reply from 8.8.8.8: bytes=32 time=4ms TTL=254
Reply from 8.8.8.8: bytes=32 time=4ms TTL=254
Reply from 8.8.8.8: bytes=32 time=4ms TTL=254

Ping statistics for 8.8.8.8:
    Packets: Sent = 4, Received = 4, Lost = 0 (0% loss),
Approximate round trip times in milli-seconds:
    Minimum = 4ms, Maximum = 5ms, Average = 4ms


Executed in 3.042282000s
```