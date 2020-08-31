# linz-port-webcam-scraper-rust

### crontab setup
```
*/5 * * * * ~/scraper/linz-port-webcam-scraper-rust --download-dir ~/images
0 0 * * * dotnet ~/linz-port-webcam-scraper-organizer/bin/Release/netcoreapp2.1/linz-port-webcam-scraper-organizer.dll ~/images/
```
