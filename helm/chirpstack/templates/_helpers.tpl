{{/*
Expand the name of the chart.
*/}}
{{- define "chirpstack.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
*/}}
{{- define "chirpstack.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart label.
*/}}
{{- define "chirpstack.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels.
*/}}
{{- define "chirpstack.labels" -}}
helm.sh/chart: {{ include "chirpstack.chart" . }}
{{ include "chirpstack.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels.
*/}}
{{- define "chirpstack.selectorLabels" -}}
app.kubernetes.io/name: {{ include "chirpstack.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use.
*/}}
{{- define "chirpstack.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "chirpstack.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
PostgreSQL hostname (internal service).
*/}}
{{- define "chirpstack.postgresql.host" -}}
{{- printf "%s-postgres" (include "chirpstack.fullname" .) }}
{{- end }}

{{/*
Redis hostname (internal service).
*/}}
{{- define "chirpstack.redis.host" -}}
{{- printf "%s-redis" (include "chirpstack.fullname" .) }}
{{- end }}

{{/*
Mosquitto hostname (internal service).
*/}}
{{- define "chirpstack.mosquitto.host" -}}
{{- printf "%s-mosquitto" (include "chirpstack.fullname" .) }}
{{- end }}

{{/*
Mosquitto MQTT server URL.
*/}}
{{- define "chirpstack.mosquitto.server" -}}
{{- printf "tcp://%s:%d" (include "chirpstack.mosquitto.host" .) (.Values.mosquitto.service.port | int) }}
{{- end }}

{{/*
Render a region TOML file, replacing the MQTT_SERVER_PLACEHOLDER with the
actual internal mosquitto address.
*/}}
{{- define "chirpstack.regionContent" -}}
{{- .content | replace "MQTT_SERVER_PLACEHOLDER" .server }}
{{- end }}
