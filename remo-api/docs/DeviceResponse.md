# DeviceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bt_mac_address** | Option<**String**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**firmware_version** | Option<**String**> |  | [optional]
**humidity_offset** | Option<**f32**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**newest_events** | Option<[**std::collections::HashMap<String, models::DeviceResponseNewestEventsValue>**](DeviceResponse_newest_events_value.md)> | The SensorValue key means 'te' = temperature, 'hu' = humidity, 'il' = illumination, 'mo' = movement. The val of 'mo' is always 1 and when movement event is captured created_at is updated. | [optional]
**online** | Option<**bool**> |  | [optional]
**serial_number** | Option<**String**> |  | [optional]
**temperature_offset** | Option<**f32**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**users** | Option<[**Vec<models::DeviceResponseUsersInner>**](DeviceResponse_users_inner.md)> | Deprecated. Do not use in new code. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


