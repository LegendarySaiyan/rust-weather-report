rust-weather-report
====================

A backend application that sends weather information via a Telegram bot

---
rust-weather-report is licensed under the [MIT license](http://opensource.org/licenses/MIT).

# How to start

### To use `weather-report`, add this to your `local_db` directory:
## credentials.json

```json
{
  "telegram_key": <YOUR_KEY>,
  "weather_key": <YOUR_KEY>
}
```

## last_update.json

```json
{
  "last_update_id": 1, #initial offset
  "was_used": false
}
```

### Then, we need to create your `forecast` and `telegram bot` clients:


1. Sign up in [weatherapi](https://www.weatherapi.com/), to get your `api_key`.
2. Create your telegram bot, using the BotFather bot.
3. Create button /command1 in your telegram bot.

### Now compile, and see the results

# How to deploy in container
```bash
docker build --progress=plain --no-cache -t weather_report .
```
Image size is 13.1 mb

### Future of the project
You can implement the webhook instead of polling. Then you need to add the `WEB API` framework, and create the docker image.
After that deploy the app image in the web server.
