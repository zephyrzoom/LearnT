# Generated by Django 2.0.3 on 2018-08-07 02:42

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('test1', '0001_initial'),
    ]

    operations = [
        migrations.AddField(
            model_name='test',
            name='upload',
            field=models.FileField(null=True, upload_to='uploads/'),
        ),
    ]
