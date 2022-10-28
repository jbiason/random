"""Resource Manager."""

import logging
import time

from threading import Condition
from threading import Lock
from threading import Thread


LOGGER = logging.getLogger(__name__)


class Resource:
    def __init__(self, manager, resources: int):
        self._manager = manager
        self._resources = resources

    def release(self):
        LOGGER.debug("Resource of %d will be returned", self._resources)
        self._manager.release(self._resources)

    @property
    def resources(self):
        return self._resources


class ResourceManager:
    def __init__(self, total_resources: int):
        self._total_resources = total_resources
        self._cond = Condition(Lock())
        LOGGER.debug("Created a manager with %d resources", self._total_resources)

    def grab(self, resources: int):
        """Grab a number of resources; if there are not enough resources
        available, blocks till they become available.
        """
        LOGGER.debug("Requesting %d resources; %d avaialble", resources, self._total_resources)
        with self._cond:
            self._cond.wait_for(lambda : self._total_resources >= resources)
            self._total_resources -= resources
        return Resource(self, resources)

    def release(self, resources: int):
        """Return the number of resources to the pool."""
        with self._cond:
            self._total_resources += resources
            self._cond.notify()
        LOGGER.debug("%d resources returned, %d available", resources, self._total_resources)


def worker(pid, resources):
    LOGGER.debug("Thread %d is holding %d resources", pid, resources.resources)
    time.sleep(10)
    LOGGER.debug("Thread %d finished, releasing %d resources", pid, resources.resources)
    resources.release()


def main():
    logging.basicConfig(level=logging.DEBUG)
    manager = ResourceManager(10)
    threads = []

    for i in range(10):
        resources = manager.grab(i+1)
        w = Thread(target=worker, args=[i, resources])
        threads.append(w)
        w.start()

    for t in threads:
        t.join()


if __name__ == '__main__':
    main()
