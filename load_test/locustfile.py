from locust import HttpUser, task
from random import randint

class LoadTest(HttpUser):
    # @task
    # def measurement_latest(self):
    #     self.client.get(f"api/measurements/latest")

    # @task
    # def measurement_latest_all(self):
    #     self.client.get(f"api/measurements/latest/all")

    @task
    def devices(self):
        self.client.get(f"api/devices")

    @task
    def sensors(self):
        self.client.get(f"api/sensors")
