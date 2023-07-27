import React, { useEffect, PropsWithChildren } from "react";

import L, { LatLngTuple, FitBoundsOptions } from "leaflet";
import "leaflet.awesome-markers";
import { MarkerProps as LMarkerProps, useMap } from "react-leaflet";
import { MapContainer, Marker as LMarker, TileLayer } from "react-leaflet";

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
  const style = {
    height: props.height,
  };

  return (
    <MapContainer
      bounds={props.bounds}
      boundsOptions={props.boundsOptions}
      center={props.center}
      zoom={13}
      scrollWheelZoom={false}
      style={style}
    >
      <TileLayer
        attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
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
