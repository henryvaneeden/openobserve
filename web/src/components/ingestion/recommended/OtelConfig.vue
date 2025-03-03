<template>
  <div class="q-pa-md">
    <div class="text-subtitle1 text-bold q-mt-md q-pl-xs">OTLP Http</div>
    <ContentCopy class="q-mt-sm" :content="getOtelHttpConfig" />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, type Ref } from "vue";
import type { Endpoint } from "@/ts/interfaces";
import ContentCopy from "@/components/CopyContent.vue";
import { useStore } from "vuex";
import { b64EncodeUnicode } from "../../../utils/zincutils";

const store = useStore();

const props = defineProps({
  currOrgIdentifier: {
    type: String,
  },
  currUserEmail: {
    type: String,
  },
});

const endpoint: any = ref({
  url: "",
  host: "",
  port: "",
  protocol: "",
  tls: "",
});

const url = new URL(store.state.API_ENDPOINT);

endpoint.value = {
  url: store.state.API_ENDPOINT,
  host: url.hostname,
  port: url.port || (url.protocol === "https:" ? "443" : "80"),
  protocol: url.protocol.replace(":", ""),
  tls: url.protocol === "https:" ? "On" : "Off",
};

const accessKey = computed(() => {
  return b64EncodeUnicode(
    `${props.currUserEmail}:${store.state.organizationData.organizationPasscode}`
  );
});

const getOtelGrpcConfig = computed(() => {
  return `exporters:
  otlp/openobserve:
      endpoint: localhost:5081
      headers:
        Authorization: "Basic ${accessKey.value}"
        organization: default
        stream-name: default
      tls:
        insecure: true
  otlp/openobserve_k8s_events:
      endpoint: localhost:5081
      headers:
        Authorization: "Basic ${accessKey.value}"
        organization: default
        stream-name: default
      tls:
        insecure: true`;
});

const getOtelHttpConfig = computed(() => {
  return `exporters:
  otlphttp/openobserve:
    endpoint: ${endpoint.value.url}/api/${props.currOrgIdentifier}/
    headers:
      Authorization: Basic ${accessKey.value}
      stream-name: default
  otlphttp/openobserve_k8s_events:
    endpoint: ${endpoint.value.url}/api/${props.currOrgIdentifier}/
    headers:
      Authorization: Basic ${accessKey.value}
      stream-name: k8s_events`;
});
</script>

<style scoped></style>
