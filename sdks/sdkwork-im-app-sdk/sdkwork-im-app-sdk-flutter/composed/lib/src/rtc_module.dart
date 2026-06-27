import 'context.dart';
import 'types.dart';
import 'package:rtc_sdk/rtc_sdk.dart';

class ImAppRtcModule {
  final ImAppSdkContext context;

  ImAppRtcModule(this.context);

  RtcDataSource get dataSource => context.rtcDataSource;

  RtcProviderMetadata describe([RtcDataSourceOptions? overrides]) {
    return dataSource.describe(overrides);
  }

  RtcProviderSelection describeSelection([RtcDataSourceOptions? overrides]) {
    return dataSource.describeSelection(overrides);
  }

  RtcProviderSupport describeProviderSupport([RtcDataSourceOptions? overrides]) {
    return dataSource.describeProviderSupport(overrides);
  }

  List<RtcProviderSupport> listProviderSupport() {
    return dataSource.listProviderSupport();
  }

  bool supportsCapability(String capability, [RtcDataSourceOptions? overrides]) {
    return dataSource.supportsCapability(capability, overrides);
  }

  bool supportsProviderExtension(String extensionKey, [RtcDataSourceOptions? overrides]) {
    return dataSource.supportsProviderExtension(extensionKey, overrides);
  }

  Future<RtcClient<TNativeClient>> createClient<TNativeClient>([
    RtcDataSourceOptions? overrides,
  ]) {
    return dataSource.createClient<TNativeClient>(overrides);
  }

  Future<ImAppJsonObject?> createProviderCallback() async {
    throw UnsupportedError(
      'RTC provider callbacks are owned by rtc_sdk and are not exposed by the app generated transport.',
    );
  }

  Future<ImAppJsonObject?> providerHealth() async {
    final support = dataSource.describeProviderSupport();
    return <String, dynamic>{
      'providerKey': support.providerKey,
      'status': support.status.name,
      'builtin': support.builtin,
      'official': support.official,
      'registered': support.registered,
    };
  }
}
