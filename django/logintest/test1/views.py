from django.http import JsonResponse
from .forms import Form
from .models import Test
from django.views.decorators.csrf import csrf_exempt
import json

@csrf_exempt
def upload_file(request):
    if request.method == 'POST':
        upfile = Form(request.POST, request.FILES)
        if upfile.is_valid():
            # file is saved
            upfile.save()
            return JsonResponse({'code':'successful'})
    else:
        upfile = Form()
    upfiles = Test.objects.all()
    return JsonResponse({'upfiles':upfiles})

@csrf_exempt
def set_text_name(request):
    body = json.loads(request.body)
    name = body['name']
    Test.objects.create(testname=name)
    return JsonResponse({'result info':'test name added'})