<!-- Copyright 2023 Zinc Labs Inc.

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http:www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License. 
-->

<!-- eslint-disable vue/x-invalid-end-tag -->
<template>

    <q-splitter
      v-model="splitterModel"
      unit="px"
      style="min-height: calc(100vh - 130px)"
    >
      <template v-slot:before>
        <q-tabs
          v-model="tabs"
          indicator-color="transparent"
        inline-label
        vertical
        >
          <q-route-tab
            default
            name="ingestLogs"
            :to="{
              name: 'ingestLogs',
              query: {
                org_identifier: store.state.selectedOrganization.identifier,
              },
            }"
            label="Logs"
            content-class="tab_content"
          />
          <q-route-tab
            name="ingestMetrics"
            :to="{
              name: 'ingestMetrics',
              query: {
                org_identifier: store.state.selectedOrganization.identifier,
              },
            }"
            label="Metrics"
            content-class="tab_content"
          />
          <q-route-tab
            name="ingestTraces"
            :to="{
              name: 'ingestTraces',
              query: {
                org_identifier: store.state.selectedOrganization.identifier,
              },
            }"
            label="Traces"
            content-class="tab_content"
          />
        </q-tabs>
      </template>

      <template v-slot:after>
        <router-view
          :title="tabs"
          :currOrgIdentifier="currOrgIdentifier"
          :currUserEmail="currentUserEmail"
          @copy-to-clipboard-fn="copyToClipboardFn"
        >
        </router-view>
      </template>
    </q-splitter>
</template>

<script lang="ts">
// @ts-ignore
import { defineComponent, ref, onBeforeMount, computed, onUpdated } from "vue";
import { useI18n } from "vue-i18n";
import { useStore } from "vuex";
import { useRouter } from "vue-router";
import { copyToClipboard, useQuasar } from "quasar";
import config from "@/aws-exports";
import segment from "@/services/segment_analytics";
import { getImageURL } from "@/utils/zincutils";

export default defineComponent({
  name: "CustomPage",
  props: {
    currOrgIdentifier: {
      type: String,
      default: "",
    },
  },
  setup() {
    const { t } = useI18n();
    const store = useStore();
    const q = useQuasar();
    const router: any = useRouter();
    const tabs = ref("");
    const currentOrgIdentifier: any = ref(
      store.state.selectedOrganization.identifier
    );
    const metricRoutes = ["prometheus", "otelCollector", "telegraf"];
    const traceRoutes = ["tracesOTLP"];
    const rumRoutes = ["frontendMonitoring"];

    onBeforeMount(() => {
      const ingestRoutes = [
        "ingestLogs",
        "ingestTraces",
        "ingestMetrics",
        "rumMonitoring",
      ];
      const logRoutes = [
        "curl",
        "fluentbit",
        "fluentd",
        "kinesisfirehose",
        "vector",
        "filebeat",
        "syslog",
        "gcpLogs",
      ];

      if (ingestRoutes.includes(router.currentRoute.value.name)) {
        router.push({
          name: router.currentRoute.value.name,
          query: {
            org_identifier: store.state.selectedOrganization.identifier,
          },
        });
        return;
      }

      if (logRoutes.includes(router.currentRoute.value.name)) {
        tabs.value = "ingestLogs";
      } else if (metricRoutes.includes(router.currentRoute.value.name)) {
        tabs.value = "ingestMetrics";
      } else if (traceRoutes.includes(router.currentRoute.value.name)) {
        tabs.value = "ingestTraces";
      } else if (ingestRoutes.includes(router.currentRoute.value.name)) {
        tabs.value = router.currentRoute.value.name;
      } else if (rumRoutes.includes(router.currentRoute.value.name)) {
        tabs.value = "rumMonitoring";
      } else if (router.currentRoute.value.name === "custom") {
        tabs.value = "ingestLogs";
        router.push({
          name: "curl",
          query: {
            org_identifier: store.state.selectedOrganization.identifier,
          },
        });
      }
    });

    onUpdated(() => {
      if (router.currentRoute.value.name === "custom") {
        tabs.value = "ingestLogs";
        router.push({
          name: "curl",
          query: {
            org_identifier: store.state.selectedOrganization.identifier,
          },
        });
      }
    });

    const copyToClipboardFn = (content: any) => {
      copyToClipboard(content.innerText)
        .then(() => {
          q.notify({
            type: "positive",
            message: "Content Copied Successfully!",
            timeout: 5000,
          });
        })
        .catch(() => {
          q.notify({
            type: "negative",
            message: "Error while copy content.",
            timeout: 5000,
          });
        });

      segment.track("Button Click", {
        button: "Copy to Clipboard",
        ingestion: router.currentRoute.value.name,
        user_org: store.state.selectedOrganization.identifier,
        user_id: store.state.userInfo.email,
        page: "Ingestion",
      });
    };

    return {
      t,
      store,
      router,
      config,
      splitterModel: ref(250),
      currentUserEmail: store.state.userInfo.email,
      currentOrgIdentifier,
      getImageURL,
      tabs,
      copyToClipboardFn,
      rumRoutes,
      traceRoutes,
      metricRoutes,
    };
  },
});
</script>

<style scoped lang="scss">
.ingestionPage {
  padding: 1.5rem 0 0;
  .head {
    padding-bottom: 1rem;
  }
  .q-tabs {
    &--vertical {
      margin: 1.5rem 1rem 0 1rem;
      .q-tab {
        justify-content: flex-start;
        padding: 0 0.6rem 0 0.6rem;
        border-radius: 0.5rem;
        margin-bottom: 0.5rem;
        text-transform: capitalize;

        &__content.tab_content {
          .q-tab {
            &__icon + &__label {
              padding-left: 0.875rem;
              font-weight: 600;
            }
          }
        }
        &--active {
          color: black;
          background-color: $accent;
        }
      }
    }
  }
}
</style>
<style lang="scss">
.ingestionPage {
  .q-tab-panel {
    padding: 0 !important;
    .tab_content {
      .q-tab__label {
        overflow: hidden !important;
        text-overflow: ellipsis !important;
        white-space: nowrap !important;
      }
    }
  }

  .q-icon > img {
    height: auto !important;
  }
}
</style>
