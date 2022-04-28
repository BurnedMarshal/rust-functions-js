import http from "k6/http";

export const options = {
  vus: 10,
  // eslint-disable-next-line sort-keys
  duration: "30s"
};

export default (): unknown =>
  http.get("http://localhost:7071/api/v1/services/01EHA20ZFP101CWMY2PYPEFVPR");
