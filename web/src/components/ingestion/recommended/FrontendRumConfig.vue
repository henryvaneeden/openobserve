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

<template>
  <div class="q-ma-md" :key="rumToken">
    <div class="title" data-test="vector-title-text">RUM</div>
    <div v-if="rumToken">
      <copy-content
        :displayContent="`Client Token: ` + rumToken"
        :content="rumToken"
      ></copy-content>

      <div class="text-h6 q-mt-md" data-test="rumweb-title-text">
        {{ t("ingestion.npmStepTitle") }}
      </div>
      <q-separator class="q-my-sm"></q-separator>

      <div class="text-subtitle1 q-mt-md" v-html="npmStep1"></div>
      <copy-content
        content="npm i @openobserve/browser-rum @openobserve/browser-logs"
      ></copy-content>

      <br />
      <div class="text-subtitle1 q-mt-md" v-html="npmStep2"></div>
      <copy-content :content="initConfiguration"></copy-content>
    </div>
    <div v-else class="q-mt-md">
      {{ t("ingestion.generateRUMTokenMessage") }}
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, onUpdated } from "vue";
import { useStore } from "vuex";
import { useI18n } from "vue-i18n";
import { getImageURL } from "../../../utils/zincutils";
import CopyContent from "../../CopyContent.vue";

export default defineComponent({
  name: "rum-web-page",
  components: {
    CopyContent,
  },
  props: {
    currOrgIdentifier: {
      type: String,
    },
    currUserEmail: {
      type: String,
    },
  },
  setup(props) {
    const store = useStore();
    const { t } = useI18n();

    const npmStep1 = ref(
      "<b>Step1: </b>Add <a href='https://www.npmjs.com/package/&#64;openobserve/browser-rum' style='color:darkorange' target='_blank'>&#64;openobserve/browser-rum</a> and <a href='https://www.npmjs.com/package/&#64;openobserve/browser-logs' style='color:darkorange' target='_blank'>&#64;openobserve/browser-logs</a> to your package.json file, or run the following command:"
    );
    const npmStep2 = ref(
      "<b>Step2: </b>Initialize the OpenObserve RUM and Logs SDKs in your application entry point (e.g. index.js or main.js)."
    );
    const rumToken = ref("");
    const defaultConfig = `
import { openobserveRum } from '@openobserve/browser-rum';
import { openobserveLogs } from '@openobserve/browser-logs';

const options = {
  clientToken: '<OPENOBSERVE_CLIENT_TOKEN>',
  applicationId: 'web-application-id',
  site: '<OPENOBSERVE_SITE>',
  service: 'my-web-application',
  env: 'production',
  version: '0.0.1',
  organizationIdentifier: '<OPENOBSERVE_ORGANIZATION_IDENTIFIER>',
  insecureHTTP: <INSECUREHTTP>,
  apiVersion: 'v1',
};

openobserveRum.init({
  applicationId: options.applicationId, // required, any string identifying your application
  clientToken: options.clientToken,
  site: options.site,
  organizationIdentifier: options.organizationIdentifier,
  service: options.service,
  env: options.env,
  version: options.version,
  trackResources: true,
  trackLongTasks: true,
  trackUserInteractions: true,
  apiVersion: options.apiVersion,
  insecureHTTP: options.insecureHTTP,
});

openobserveLogs.init({
  clientToken: options.clientToken,
  site: options.site,
  organizationIdentifier: options.organizationIdentifier,
  forwardErrorsToLogs: true,
  insecureHTTP: options.insecureHTTP,
  apiVersion: options.apiVersion,
});

// You can set a user context
// openobserveRum.setUser({
//   id: "<ID>",
//   name: "<NAME>",
//   email: "<EMAIL>",
// });

openobserveRum.startSessionReplayRecording();`;
    const initConfiguration = ref(defaultConfig);

    onMounted(() => {
      if (store.state.organizationData.rumToken) {
        rumToken.value = store.state.organizationData.rumToken.rum_token;
      }
    });

    onUpdated(() => {
      if (store.state.organizationData.rumToken) {
        replaceStaticValues();
      }
    });

    const replaceStaticValues = () => {
      rumToken.value = store.state.organizationData.rumToken.rum_token;
      let configData = defaultConfig;
      configData = configData.replace(
        /<OPENOBSERVE_CLIENT_TOKEN>/g,
        rumToken.value
      );

      configData = configData.replace(
        /<OPENOBSERVE_SITE>/g,
        store.state.API_ENDPOINT.replace("https://", "")
          .replace("http://", "")
          .replace(/\/$/, "")
      );

      configData = configData.replace(
        /<OPENOBSERVE_ORGANIZATION_IDENTIFIER>/g,
        store.state.selectedOrganization.identifier
      );

      if (store.state.API_ENDPOINT.indexOf("https://") > -1) {
        configData = configData.replace(/<INSECUREHTTP>/g, "false");
      } else {
        configData = configData.replace(/<INSECUREHTTP>/g, "true");
      }

      initConfiguration.value = configData;
    };

    return {
      t,
      store,
      getImageURL,
      rumToken,
      npmStep1,
      npmStep2,
      initConfiguration,
      replaceStaticValues,
    };
  },
  computed: {
    checkRUMToken() {
      return this.store.state.organizationData.rumToken;
    },
  },
  watch: {
    checkRUMToken() {
      this.rumToken = this.store.state.organizationData.rumToken.key;
      this.replaceStaticValues();
    },
  },
});
</script>

<style scoped lang="scss">
.tabContent {
  background-color: rgba(136, 136, 136, 0.103);
  // tab content bg color
  padding: 10px 20px;
  border-radius: 0.5rem;
  &__head {
    justify-content: space-between;
    text-transform: uppercase;
    align-items: center;
    display: flex;
    .title {
      line-height: 1rem;
      font-weight: 600;
    }
    .copy_action {
      .q-btn {
        // background-color: white;
      }
    }
  }
  pre {
    white-space: pre-wrap;
    word-wrap: break-word;
    margin: 0;
  }
}
</style>
