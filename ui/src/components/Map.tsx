import React, { Component } from "react";

import L, { LatLngTuple, FitBoundsOptions } from "leaflet";
import "leaflet.awesome-markers";
import { MarkerProps as LMarkerProps } from "react-leaflet";
import { MapContainer, Marker as LMarker, TileLayer } from "react-leaflet";

interface IProps {
  height: number;
  center?: [number, number];
  bounds?: LatLngTuple[];
  boundsOptions?: FitBoundsOptions;
}

interface IState {
  map?: L.Map;
}

class Map extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  setMap = (map: L.Map) => {
    this.setState(
      {
        map: map,
      },
      () => {
        // This is needed as setMap is called after the map has been created.
        // There is a small amount of time where componentDidUpdate can't update
        // the map with the new center because setMap hasn't been called yet.
        // In such case, the map would never update to the new center.
        if (this.props.center !== undefined) {
          map.panTo(this.props.center);
        }

        if (this.props.bounds !== undefined) {
          map.fitBounds(this.props.bounds, this.props.boundsOptions);
        }
      },
    );
  };

  componentDidUpdate(oldProps: IProps) {
    if (this.props === oldProps) {
      return;
    }

    if (this.state.map) {
      if (this.props.center !== undefined) {
        this.state.map.flyTo(this.props.center);
      }

      if (this.props.bounds !== undefined) {
        this.state.map.flyToBounds(this.props.bounds, this.props.boundsOptions);
      }
    }
  }

  render() {
    const style = {
      height: this.props.height,
    };

    return (
      <MapContainer
        bounds={this.props.bounds}
        boundsOptions={this.props.boundsOptions}
        center={this.props.center}
        zoom={13}
        scrollWheelZoom={false}
        animate={true}
        style={style}
        whenCreated={this.setMap}
      >
        <TileLayer
          attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        {this.props.children}
      </MapContainer>
    );
  }
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

export class Marker extends Component<MarkerProps> {
  render() {
    const { faIcon, color, position, ...otherProps } = this.props;

    const iconMarker = L.AwesomeMarkers.icon({
      icon: faIcon,
      prefix: "fa",
      markerColor: color,
    });

    return (
      <LMarker icon={iconMarker} position={position} {...otherProps}>
        {this.props.children}
      </LMarker>
    );
  }
}

export default Map;
