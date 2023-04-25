from django.db import models


class Parent(models.Model):
    name = models.CharField(max_length=256)


class Child(models.Model):
    name = models.CharField(max_length=256)
    parent = models.ForeignKey(Parent, on_delete=models.CASCADE)
    proportion = models.FloatField()
