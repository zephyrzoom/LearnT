from django.http import JsonResponse
from .models import Interface
import sys
# Create your views here.
def get_interface(request, productid):
    interface = Interface.objects.filter(url=productid)
    print(sys.modules['django'])
    if interface.exists():
        return JsonResponse({"value":interface[0].value})
    else:
        return JsonResponse({"error":"wrong id"})