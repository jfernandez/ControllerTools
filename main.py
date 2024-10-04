import asyncio
import asyncio.subprocess
import logging
import typing

import decky # type: ignore
import settings  # type: ignore

HOME_DIR = decky.DECKY_HOME
PARENT_DIR = decky.DECKY_PLUGIN_DIR

logger = decky.logger
logger.setLevel(logging.DEBUG)
logging.info(f"ControllerTools main.py https://github.com/alphamercury/ControllerTools")

logger.info("[backend] Settings path: {}".format(decky.DECKY_PLUGIN_SETTINGS_DIR))
settings = settings.SettingsManager(name="settings", settings_directory=decky.DECKY_PLUGIN_SETTINGS_DIR)
settings.read()

class Plugin:
    BACKEND_PROC: typing.Optional[asyncio.subprocess.Process] = None

    @classmethod
    async def _main(cls):
        cls.BACKEND_PROC = await asyncio.subprocess.create_subprocess_exec(
            PARENT_DIR + "/bin/backend",
            f"{decky.DECKY_PLUGIN_SETTINGS_DIR}/settings.json",
            decky.DECKY_PLUGIN_LOG,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )

    @classmethod
    async def _unload(cls):
        if cls.BACKEND_PROC is not None:
            cls.BACKEND_PROC.terminate()
            await cls.BACKEND_PROC.wait()
            cls.BACKEND_PROC = None

    @classmethod
    async def settings_read(cls):
        logger.info("[backend] Reading settings")
        return settings.read()

    @classmethod
    async def settings_commit(cls):
        logger.info("[backend] Saving settings")
        return settings.commit()

    @classmethod
    async def settings_getSetting(cls, key: str, defaults: typing.Any):
        logger.info("[backend] Get {}".format(key))
        return settings.getSetting(key, defaults)

    @classmethod
    async def settings_setSetting(cls, key: str, value: typing.Any):
        logger.info("[backend] Set {}: {}".format(key, value))
        return settings.setSetting(key, value)
