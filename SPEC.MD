# General Paired Transfer Protocol - Specification

Specification and implementation reference for GPTP.

## Byte Allocation Table

| Bytes         | Info Type      |
|:-------------:|:--------------:|
| 0 to 2        | Message Type   |
| 3 and 4       | Message Length |
| 76 up to 1099 | Message Body   |

## Message Types

REQ - Request to start data exchange.
ACP - Accept a REQ.
DNY - Deny a REQ.
TXT - A text message or part of one.
RQF - Request to send file. Body contains file name.
ACF - Accept a RQF.
DNF - Deny a RQF.
FLE - A file or part of one.