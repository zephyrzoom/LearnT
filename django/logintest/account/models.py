from django.db import models
from django.contrib.auth.models import User

class App(models.Model):
    app_name = models.CharField(max_length=50)
    description = models.CharField(max_length=50)