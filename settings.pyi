import typing


class SettingsManager:
    def __init__(self, name: str, settings_directory: str | None = None) -> None: ...
    def read(self) -> typing.Any: ...
    def commit(self) -> typing.Any: ...
    def getSetting(self, key: str, defaults: typing.Any) -> typing.Any: ...
    def setSetting(self, key: str, value: typing.Any) -> typing.Any: ...