from django.db import models

# Create your models here.
class Interface(models.Model):
    url = models.CharField(max_length=20)
    value = models.CharField(max_length=20, null=True)