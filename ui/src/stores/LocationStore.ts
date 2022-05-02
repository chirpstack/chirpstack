import { EventEmitter } from "events";
import { notification } from "antd";

class LocationStore extends EventEmitter {
  getLocation = (callbackFunc: (loc: [number, number]) => void) => {
    if (!navigator.geolocation) {
      notification.error({
        message: "The browser does not support geolocation",
        duration: 3,
      });
      return;
    }

    navigator.geolocation.getCurrentPosition(
      (p: GeolocationPosition) => {
        callbackFunc([p.coords.latitude, p.coords.longitude]);
      },
      (e: GeolocationPositionError) => {
        notification.error({
          message: e.message,
          duration: 3,
        });
      },
      {
        timeout: 3000,
      },
    );
  };
}

const locationStore = new LocationStore();
export default locationStore;
