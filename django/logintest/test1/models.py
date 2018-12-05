from django.db import models

# Create your models here.
class Test(models.Model):
    NAME = (
        ('a','aaa'),
        ('b','bbb'),
    )
    testname = models.CharField(max_length=1, choices=NAME)
    upload = models.FileField(upload_to='uploads/', null=True)