# SMDR Logger

Created for Avaya phone systems, however it should work on most systems.
The purpose is to only dispatch/send the call information once the call is terminated.
This means one `ActivePhoneCall` will contain every `SMDRRecord` from call start to end
(including transfers).

https://documentation.avaya.com/bundle/AdministeringAvayaIPOfficePlatformManagerR11_1/page/SMDR_Fields.html

## Webhook Data
```json
{
    "call_id": "1163546",
    "start": "2025/02/20 11:34:40",
    "records": [
        {
            "start": "2025/02/20 11:34:40",
            "duration": 32,
            "ring": 0,
            "caller": "07555012345@xx.xxx.xx.xxx",
            "direction": "I",
            "called": "706",
            "dialled": "01515551234",
            "account": "134",
            "is_internal": false,
            "call_id": "1163546",
            "continued": true,
            "party_1_device": "V9510",
            "party_1_name": "VM Channel 10"
        },
        {
            "start": "2025/02/20 11:35:12",
            "duration": 2336,
            "ring": 1,
            "caller": "07555012345@xx.xxx.xx.xxx",
            "direction": "I",
            "called": "700",
            "dialled": "01515551234",
            "account": "134",
            "is_internal": false,
            "call_id": "1163546",
            "continued": false,
            "party_1_device": "E627",
            "party_1_name": "Employee"
        }
    ]
}
```

> Note: SIP Trunks will include the IP address in caller field.
> The `caller` may also be `anonymous@anonymous.invalid` if the [caller ID was withheld](https://documentation.avaya.com/bundle/IPOfficeWebManagerR12/page/SIP_Anonymous_Calls.html).

## Environment Variables

| Key                       | Example                             | Description                                              | Required |
|---------------------------|-------------------------------------|----------------------------------------------------------|----------|
| SMDR_ADDR                 | `192.168.1.2:8123`                  | The Phone System SMDR port.                              | Yes      |
| SMDR_WEBHOOK_URL          | `https://...`                       | Target webhook URL.                                      | Yes      |
| SMDR_WEBHOOK_KEY          | `token`                             | Sent as `Authorization` header for `SMDR_WEBHOOK_URL`.   | Yes      |
| SMDR_WEBHOOK_MAX_RETRIES  | Default: `25`                       | The max amount of retries for sending a call webhook.    | No       |
| SMDR_WEBHOOK_RETRY_DELAY  | Default: `30`                       | Amount of seconds to delay for webhook retries.          | No       |
| SMDR_SENTRY_DSN           | `https://abc123@sentry.com/...`     | A Sentry DSN to send errors to.                          | Yes      |
| SMDR_SENTRY_CRON_URL      | `https://sentry.com/api/?/cron/...` | A Sentry CRON HTTP URL to act as health-checks.          | No       |
| SMDR_SENTRY_CRON_INTERVAL | Default: `180`                      | Amount of seconds interval between Sentry CRON requests. | No       |