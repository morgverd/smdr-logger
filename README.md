# SMDR Logger

Created for Avaya phone systems, however it should work on most systems.
The purpose is to only dispatch/send the call information once the call is terminated.
This means one `ActivePhoneCall` will contain every `SMDRRecord` from call start to end
(including transfers).

https://documentation.avaya.com/bundle/AdministeringAvayaIPOfficePlatformManagerR11_1/page/SMDR_Fields.html