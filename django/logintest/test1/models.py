from django.db import models

# Create your models here.
class Test(models.Model):
    testname = models.CharField(max_length=23)