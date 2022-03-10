import { toApiServiceMetadata } from "@pagopa/io-functions-commons/dist/src/utils/service_metadata";
import { RetrievedService } from "@pagopa/io-functions-commons/dist/src/models/service";
import { ServicePublic } from "../generated/definitions/ServicePublic";
import {
  NotificationChannel,
  NotificationChannelEnum
} from "../generated/definitions/NotificationChannel";

export const serviceAvailableNotificationChannels = (
  retrievedService: RetrievedService
): ReadonlyArray<NotificationChannel> => {
  if (retrievedService.requireSecureChannels) {
    return [NotificationChannelEnum.WEBHOOK];
  }
  return [NotificationChannelEnum.EMAIL, NotificationChannelEnum.WEBHOOK];
};

export const retrievedServiceToPublic = (
  retrievedService: RetrievedService
): ServicePublic => ({
  available_notification_channels: serviceAvailableNotificationChannels(
    retrievedService
  ),
  department_name: retrievedService.departmentName,
  organization_fiscal_code: retrievedService.organizationFiscalCode,
  organization_name: retrievedService.organizationName,
  service_id: retrievedService.serviceId,
  service_metadata: retrievedService.serviceMetadata
    ? toApiServiceMetadata(retrievedService.serviceMetadata)
    : undefined,
  service_name: retrievedService.serviceName,
  version: retrievedService.version
});
