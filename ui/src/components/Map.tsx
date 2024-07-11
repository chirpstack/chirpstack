import type { PropsWithChildren } from "react";
import { useEffect, useState } from "react";

import type { LatLngTuple, FitBoundsOptions } from "leaflet";
import L from "leaflet";
import "leaflet.awesome-markers";
import type { MarkerProps as LMarkerProps } from "react-leaflet";
import { useMap } from "react-leaflet";
import { MapContainer, Marker as LMarker, TileLayer } from "react-leaflet";

import InternalStore from "../stores/InternalStore";

interface IProps {
  height: number;
  center?: [number, number];
  bounds?: LatLngTuple[];
  boundsOptions?: FitBoundsOptions;
}

function MapControl(props: { center?: [number, number]; bounds?: LatLngTuple[]; boundsOptions?: FitBoundsOptions }) {
  const map = useMap();

  useEffect(() => {
    if (map === undefined) {
      return;
    }

    if (props.center !== undefined) {
      map.flyTo(props.center);
    }

    if (props.bounds !== undefined) {
      map.flyToBounds(props.bounds, props.boundsOptions);
    }
  });

  return null;
}

function Map(props: PropsWithChildren<IProps>) {
  const [tileserver, setTileserver] = useState<string>("");
  const [attribution, setAttribution] = useState<string>("");

  useEffect(() => {
    const updateMapProperties = () => {
      InternalStore.settings(v => {
        setTileserver(v.getTileserverUrl());
        setAttribution(v.getMapAttribution());
      });
    };

    InternalStore.on("change", updateMapProperties);
    updateMapProperties();

    return () => {
      InternalStore.removeListener("change", updateMapProperties);
    };
  }, [props]);

  const style = {
    height: props.height,
  };

  if (attribution === "" || tileserver === "") {
    return null;
  }

  return (
    <MapContainer
      bounds={props.bounds}
      boundsOptions={props.boundsOptions}
      center={props.center}
      zoom={13}
      scrollWheelZoom={false}
      style={style}
    >
      <TileLayer attribution={attribution} url={tileserver} />
      {props.children}
      <MapControl bounds={props.bounds} boundsOptions={props.boundsOptions} center={props.center} />
    </MapContainer>
  );
}

export type MarkerColor =
  | "red"
  | "darkred"
  | "orange"
  | "green"
  | "darkgreen"
  | "blue"
  | "purple"
  | "darkpurple"
  | "cadetblue"
  | undefined;

interface MarkerProps extends LMarkerProps {
  position: [number, number];
  faIcon: string;
  color: MarkerColor;
}

export function Marker(props: MarkerProps) {
  const { faIcon, color, position, ...otherProps } = props;

  const iconMarker = L.AwesomeMarkers.icon({
    icon: faIcon,
    prefix: "fa",
    markerColor: color,
  });

  return (
    <LMarker icon={iconMarker} position={position} {...otherProps}>
      {props.children}
    </LMarker>
  );
}

export default Map;
