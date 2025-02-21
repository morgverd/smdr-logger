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
  "start": "2025/02/21 11:01:18",
  "caller": {
    "dialled": "01515551234",
    "caller": "07555012345@xx.xxx.xx.xxx",
    "party_2_device": "T9100",
    "party_2_name": "Line 100.1"
  },
  "records": [
    {
      "duration": 37,
      "ring": 0,
      "direction": "I",
      "called": "706",
      "account": "",
      "is_internal": false,
      "continued": true,
      "party_1_device": "V9512",
      "party_1_name": "VM Channel 12",
      "hold_time": 0,
      "park_time": 0
    },
    {
      "duration": 826,
      "ring": 1,
      "direction": "I",
      "called": "700",
      "account": "",
      "is_internal": false,
      "continued": false,
      "party_1_device": "E627",
      "party_1_name": "Employee",
      "hold_time": 5,
      "park_time": 0
    }
  ]
}
```

> SIP Trunks will include the IP address in `caller` field.
> The `caller` may also be `anonymous@anonymous.invalid` if the [caller ID was withheld](https://documentation.avaya.com/bundle/IPOfficeWebManagerR12/page/SIP_Anonymous_Calls.html).

> The `duration` field does not include `hold_time` or `park_time`.

## Environment Variables

| Key                       | Example                             | Description                                              | Required |
|---------------------------|-------------------------------------|----------------------------------------------------------|----------|
| SMDR_ADDR                 | `192.168.1.2:8123`                  | The Phone System SMDR port.                              | Yes      |
| SMDR_WEBHOOK_URL          | `https://...`                       | Target webhook URL.                                      | Yes      |
| SMDR_WEBHOOK_KEY          | `token`                             | Sent as `Authorization` header for `SMDR_WEBHOOK_URL`.   | Yes      |
| SMDR_WEBHOOK_MAX_RETRIES  | Default: `25`                       | The max amount of retries for sending a call webhook.    | No       |
| SMDR_WEBHOOK_RETRY_DELAY  | Default: `30`                       | Amount of seconds to delay for webhook retries.          | No       |
| SMDR_SENTRY_DSN           | `https://abc123@sentry.com/...`     | A Sentry DSN to send errors to.                          | No       |
| SMDR_SENTRY_CRON_URL      | `https://sentry.com/api/?/cron/...` | A Sentry CRON HTTP URL to act as health-checks.          | No       |
| SMDR_SENTRY_CRON_INTERVAL | Default: `180`                      | Amount of seconds interval between Sentry CRON requests. | No       |