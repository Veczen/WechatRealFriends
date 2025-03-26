[简体中文](README.md) | [English](README_EN.md)

### WechatRealFriends
----

A tool to instantly check your WeChat friendship status - find out if anyone has secretly deleted or blocked you. Based on the WeChat iPad protocol.

----
# Features:
**Quickly detect one-way friendships in your WeChat and automatically add them to a tag (which can be later cleared in bulk on PC). Theoretically supports checking thousands of friends.**
> Note: This software won't disturb your friends - they won't receive any notifications.

# Usage Guide:
- Download the latest build version from releases [Go to releases page](https://github.com/StrayMeteor3337/WechatRealFriends/releases/)

- After extracting, follow the instructions in the txt document

**Important Notes:**
1. **If you get a numeric verification code**, before scanning the QR code, please change your WeChat language to English in your phone's WeChat Settings -> General -> Language. Then make sure to **LOG OUT** (not just close) of WeChat on your phone and log back in before scanning the QR code again.
2. The authorized device type is iPad - please select this correctly if prompted.

# Screenshots:
![Cover](https://gitee.com/StrayMeteor3337/strayImg/raw/master/wrf-cover.jpg)

![Login Interface](https://gitee.com/StrayMeteor3337/strayImg/raw/master/wrf-login.jpg)

![Main Interface](https://gitee.com/StrayMeteor3337/strayImg/raw/master/wrf.jpg)

# Additional Information:
The WeChat iPad protocol implementation used in this software was not written by me, and **I don't have access to the protocol's source code**. The software uses a compiled version. The protocol has many other features which you can explore through the SwaggerUI interface.

# Acknowledgments:
- ZogeMung - Contributed code to fix the issue of total friend count being less than actual
- OutfitPure - Provided the method to bypass WeChat's numeric verification code during scan login 