// Copyright 2023 Zinc Labs Inc.

//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at

//      http:www.apache.org/licenses/LICENSE-2.0

//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

import { reactive } from "vue";

const defaultObject = {
  data: {
    datetime: {
      startTime: 0,
      endTime: 0,
      relativeTimePeriod: "15m",
      valueType: "relative",
    },
  },
};

let performanceState = reactive(Object.assign({}, defaultObject));

const usePerformance = () => {
  const resetSessionState = () => {
    // delete searchObj.data;
    performanceState = reactive(Object.assign({}, defaultObject));
  };

  return { performanceState, resetSessionState };
};

export default usePerformance;
